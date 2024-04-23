import {
  Button,
  Group,
  useMantineTheme,
  Fieldset,
} from "@mantine/core";
import { invoke } from "@tauri-apps/api/tauri";

const GameCleanup = () => {
  const theme = useMantineTheme();
  const handleCleanup = async (cleanType: string) => {
    try {
      if (cleanType === "arrangeMapsFolder") {
        // TODO Progress bar? Finish/Error notification?
        await invoke("arrange_maps_folder");
      }
    } catch (error) {
      console.error("Error opening folder:", error);
    }
  };

  return (
    <Fieldset
      legend="Game Cleanup"
      style={{
        border: `${theme.colors.dark[4]} 1px solid`,
        borderRadius: "8px",
        padding: "10px",
      }}
    >
      <Group>
        <Button onClick={() => handleCleanup("arrangeMapsFolder")}>
          Sort Community Maps Folder
        </Button>
        {/* <Button onClick={() => handleCleanup("deleteCommunityMaps")}>
          Delete Community Maps
        </Button> */}
      </Group>
    </Fieldset>
  );
};

export default GameCleanup;
