import { useParams } from "react-router-dom";

function EpisodeListPage() {
  const { id } = useParams();

  return (
    <div>
      <h1>エピソード一覧 (Podcast ID: {id})</h1>
    </div>
  );
}

export default EpisodeListPage;
