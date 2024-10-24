import { createApp } from "vue";
import App from "./App.vue";
import "./style.scss";

import "vuetify/dist/vuetify.min.css";
import "@mdi/font/css/materialdesignicons.min.css";
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";
import { lightTheme, darkTheme } from "./theme";

const vuetify = createVuetify({
  components,
  directives,
  icons: {
    defaultSet: "mdi",
  },
  theme: {
    themes: { light: lightTheme, dark: darkTheme },
    defaultTheme: "dark",
  },
});

createApp(App).use(vuetify).mount("#app");
