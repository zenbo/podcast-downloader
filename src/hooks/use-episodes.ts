import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { listEpisodes, checkNewEpisodes } from "@/services/episode";
import { podcastKeys } from "@/hooks/use-podcasts";

export const episodeKeys = {
  all: ["episodes"] as const,
  list: (podcastId: number) =>
    [...episodeKeys.all, "list", podcastId] as const,
};

export function useEpisodes(podcastId: number) {
  return useQuery({
    queryKey: episodeKeys.list(podcastId),
    queryFn: () => listEpisodes(podcastId),
  });
}

export function useCheckNewEpisodes() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (podcastId: number) => checkNewEpisodes(podcastId),
    onSuccess: (_data, podcastId) => {
      queryClient.invalidateQueries({
        queryKey: episodeKeys.list(podcastId),
      });
      queryClient.invalidateQueries({ queryKey: podcastKeys.all });
    },
  });
}
