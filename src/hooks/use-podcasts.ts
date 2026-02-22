import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import {
  listPodcasts,
  registerPodcast,
  deletePodcast,
} from "@/services/podcast";
import { checkAllNew } from "@/services/episode";

export const podcastKeys = {
  all: ["podcasts"] as const,
  list: () => [...podcastKeys.all, "list"] as const,
};

export function usePodcasts() {
  return useQuery({
    queryKey: podcastKeys.list(),
    queryFn: listPodcasts,
  });
}

export function useRegisterPodcast() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (url: string) => registerPodcast(url),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: podcastKeys.all });
    },
  });
}

export function useDeletePodcast() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (podcastId: number) => deletePodcast(podcastId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: podcastKeys.all });
    },
  });
}

export function useCheckAllNew() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () => checkAllNew(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: podcastKeys.all });
    },
  });
}
