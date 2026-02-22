import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { FolderOpen, Plus, Trash2, Loader2 } from "lucide-react";
import { toast } from "sonner";

import type { AppSettings, CharacterReplacement } from "@/types";
import { selectFolder } from "@/services/settings";
import { useSettings, useUpdateSettings } from "@/hooks/use-settings";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card } from "@/components/ui/card";
import { Header } from "@/components/common/Header";

function SettingsPage() {
  const navigate = useNavigate();
  const { data: settings, isLoading } = useSettings();
  const mutation = useUpdateSettings();

  const [downloadDir, setDownloadDir] = useState<string | null>(null);
  const [replacements, setReplacements] = useState<CharacterReplacement[]>([]);
  const [fallback, setFallback] = useState("_");
  const [initialized, setInitialized] = useState(false);

  useEffect(() => {
    if (settings && !initialized) {
      setDownloadDir(settings.downloadDir);
      setReplacements(settings.characterReplacements);
      setFallback(settings.fallbackReplacement);
      setInitialized(true);
    }
  }, [settings, initialized]);

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

  function handleSave() {
    const newSettings: AppSettings = {
      downloadDir,
      characterReplacements: replacements,
      fallbackReplacement: fallback,
    };

    mutation.mutate(newSettings, {
      onSuccess: () => {
        toast.success("設定を保存しました");
        navigate("/");
      },
      onError: (err) => {
        toast.error(String(err));
      },
    });
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
      <Header backTo="/" />

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

          {/* アクションボタン */}
          <div className="flex items-center justify-end gap-2 pt-4 border-t">
            <Button variant="outline" onClick={() => navigate("/")}>
              キャンセル
            </Button>
            <Button onClick={handleSave} disabled={mutation.isPending}>
              {mutation.isPending && (
                <Loader2 className="mr-1.5 h-4 w-4 animate-spin" />
              )}
              保存
            </Button>
          </div>
        </div>
      </main>
    </div>
  );
}

export default SettingsPage;
