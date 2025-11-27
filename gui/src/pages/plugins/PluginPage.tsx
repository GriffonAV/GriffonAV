import { useParams } from "react-router-dom";

export default function PluginPage() {
  const { pid } = useParams();
  return <div>Plugin UI for PID: {pid}</div>;
}
