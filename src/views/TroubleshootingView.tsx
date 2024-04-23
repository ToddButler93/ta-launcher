// TroubleshootView.tsx

import { Space } from "@mantine/core";
import LogFileCleanup from "../components/LogFileCleanup";
import GameCleanup from "../components/GameCleanup";

const TroubleshootView = () => {

  return (
    <div>
      <LogFileCleanup />
      <Space h="md" />
      <GameCleanup />
    </div>
  );
};

export default TroubleshootView;
