// MUI's own default dark theme — a deliberate reset, not a port of the
// live app's felt/card palette. This library redesigns the look from
// Material's defaults rather than carrying the old one forward.
import { createTheme } from "@mui/material/styles";

export const theme = createTheme({
  palette: {
    mode: "dark",
  },
});
