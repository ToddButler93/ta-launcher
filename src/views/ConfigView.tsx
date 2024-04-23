// ConfigView.tsx

import { useState } from "react";
import { Paper, Space, Switch, useMantineTheme } from "@mantine/core";
import DirectoryShortcuts from "../components/DirectoryShortcuts";
import ConfigToggleSection from "../components/ConfigToggleSection";
import GameCleanup from "../components/GameCleanup";

const ConfigView = () => {
  const theme = useMantineTheme();
  const [isAdvancedMode, setAdvancedMode] = useState(false);

  const handleModeToggle = () => {
    setAdvancedMode(!isAdvancedMode);
  };

  return (
    <div>
      <DirectoryShortcuts />
      <Space h="md" />

      <Paper
        style={{
          border: `${theme.colors.dark[4]} 1px solid`,
          borderRadius: "8px",
          padding: "10px",
        }}
      >
        <Switch
          label={isAdvancedMode ? "Advanced Mode" : "Simple Mode"}
          checked={isAdvancedMode}
          onChange={handleModeToggle}
          size="md"
        />
        <Space h="md" />
        <ConfigToggleSection isAdvancedMode={isAdvancedMode} />
      </Paper>
    </div>
  );
};

export default ConfigView;
