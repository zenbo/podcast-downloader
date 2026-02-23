import { useEffect, useRef, useState } from "react";
import { Check, FolderOpen, Loader2, Plus, Trash2 } from "lucide-react";
import { toast } from "sonner";

import type { AppSettings, CharacterReplacement } from "@/types";
import { selectFolder, updateSettings } from "@/services/settings";
import { useSettings, useUpdateSettings } from "@/hooks/use-settings";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card } from "@/components/ui/card";
import { Header } from "@/components/common/Header";

function SettingsPage() {
  const { data: settings, isLoading } = useSettings();
  const mutation = useUpdateSettings();

  const [downloadDir, setDownloadDir] = useState<string | null>(null);
  const [replacements, setReplacements] = useState<CharacterReplacement[]>([]);
  const [fallback, setFallback] = useState("_");
  const [initialized, setInitialized] = useState(false);
  const [saveStatus, setSaveStatus] = useState<"idle" | "saving" | "saved">(
    "idle",
  );

  const saveTimeoutRef = useRef<ReturnType<typeof setTimeout>>(undefined);
  const savedStatusTimeoutRef = useRef<ReturnType<typeof setTimeout>>(undefined);
  const prevSettingsRef = useRef("");
  const currentSettingsRef = useRef("");

  useEffect(() => {
    if (settings && !initialized) {
      setDownloadDir(settings.downloadDir);
      setReplacements(settings.characterReplacements);
      setFallback(settings.fallbackReplacement);
      setInitialized(true);
      const json = JSON.stringify(settings);
      prevSettingsRef.current = json;
      currentSettingsRef.current = json;
    }
  }, [settings, initialized]);

  // 自動保存（デバウンス 500ms）
  useEffect(() => {
    if (!initialized) return;

    const current: AppSettings = {
      downloadDir,
      characterReplacements: replacements,
      fallbackReplacement: fallback,
    };
    const json = JSON.stringify(current);
    currentSettingsRef.current = json;

    if (json === prevSettingsRef.current) return;

    clearTimeout(saveTimeoutRef.current);
    saveTimeoutRef.current = setTimeout(() => {
      setSaveStatus("saving");
      mutation.mutate(current, {
        onSuccess: () => {
          prevSettingsRef.current = json;
          setSaveStatus("saved");
          clearTimeout(savedStatusTimeoutRef.current);
          savedStatusTimeoutRef.current = setTimeout(
            () => setSaveStatus("idle"),
            2000,
          );
        },
        onError: (err) => {
          toast.error(`設定の保存に失敗しました: ${String(err)}`);
          setSaveStatus("idle");
        },
      });
    }, 500);

    return () => clearTimeout(saveTimeoutRef.current);
  }, [initialized, downloadDir, replacements, fallback]);

  // アンマウント時に未保存の変更をフラッシュ
  useEffect(() => {
    return () => {
      clearTimeout(saveTimeoutRef.current);
      clearTimeout(savedStatusTimeoutRef.current);
      if (
        currentSettingsRef.current &&
        currentSettingsRef.current !== prevSettingsRef.current
      ) {
        const pending = JSON.parse(currentSettingsRef.current) as AppSettings;
        updateSettings(pending).catch(() => {});
      }
    };
  }, []);

  async function handleSelectFolder() {
    try {
      const path = await selectFolder();
      if (path) {
        setDownloadDir(path);
      }
    } catch (err) {
      toast.error(String(err));
    }
  }

  function addRule() {
    setReplacements((prev) => [...prev, { before: "", after: "" }]);
  }

  function removeRule(index: number) {
    setReplacements((prev) => prev.filter((_, i) => i !== index));
  }

  function updateRule(
    index: number,
    field: keyof CharacterReplacement,
    value: string,
  ) {
    setReplacements((prev) =>
      prev.map((r, i) => (i === index ? { ...r, [field]: value } : r)),
    );
  }

  if (isLoading) {
    return (
      <div className="flex flex-col h-screen">
        <Header backTo="/" />
        <div className="flex-1 flex items-center justify-center">
          <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-screen">
      <Header backTo="/">
        {saveStatus === "saving" && (
          <span className="flex items-center gap-1 text-sm text-muted-foreground">
            <Loader2 className="h-3.5 w-3.5 animate-spin" />
            保存中...
          </span>
        )}
        {saveStatus === "saved" && (
          <span className="flex items-center gap-1 text-sm text-muted-foreground">
            <Check className="h-3.5 w-3.5" />
            保存済み
          </span>
        )}
      </Header>

      <main className="flex-1 overflow-auto p-4">
        <div className="max-w-2xl mx-auto space-y-6">
          {/* ダウンロードフォルダ */}
          <section>
            <Label className="mb-2 block text-base font-semibold">
              ダウンロード先フォルダ
            </Label>
            <div className="flex items-center gap-2">
              <Input
                readOnly
                value={downloadDir ?? ""}
                placeholder="未設定"
                className="flex-1"
              />
              <Button variant="outline" onClick={handleSelectFolder}>
                <FolderOpen className="mr-1.5 h-4 w-4" />
                選択
              </Button>
            </div>
          </section>

          {/* 文字置換ルール */}
          <section>
            <Label className="mb-2 block text-base font-semibold">
              文字置換ルール
            </Label>
            <p className="text-sm text-muted-foreground mb-3">
              ファイル名に使えない文字を置換するルールです。上から順に適用されます。
            </p>
            <div className="space-y-2">
              {replacements.map((rule, index) => (
                <Card key={index} className="flex items-center gap-2 p-2">
                  <Input
                    value={rule.before}
                    onChange={(e) => updateRule(index, "before", e.target.value)}
                    placeholder="置換前"
                    className="w-24"
                  />
                  <span className="text-muted-foreground">→</span>
                  <Input
                    value={rule.after}
                    onChange={(e) => updateRule(index, "after", e.target.value)}
                    placeholder="置換後"
                    className="w-24"
                  />
                  <Button
                    variant="ghost"
                    size="icon"
                    aria-label="ルールを削除"
                    onClick={() => removeRule(index)}
                  >
                    <Trash2 className="h-4 w-4" />
                  </Button>
                </Card>
              ))}
            </div>
            <Button
              variant="outline"
              size="sm"
              className="mt-2"
              onClick={addRule}
            >
              <Plus className="mr-1.5 h-4 w-4" />
              ルールを追加
            </Button>
          </section>

          {/* フォールバック置換文字 */}
          <section>
            <Label
              htmlFor="fallback"
              className="mb-2 block text-base font-semibold"
            >
              フォールバック置換文字
            </Label>
            <p className="text-sm text-muted-foreground mb-2">
              上記ルールに該当しない禁止文字の置換先です。
            </p>
            <Input
              id="fallback"
              value={fallback}
              onChange={(e) => setFallback(e.target.value)}
              className="w-24"
            />
          </section>

        </div>
      </main>
    </div>
  );
}

export default SettingsPage;
