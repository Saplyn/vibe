import { createApp } from "vue";
import App from "./App.vue";

import PrimeVue from "primevue/config";
import Aura from "@primeuix/themes/aura";
import { definePreset } from "@primeuix/themes";
import ToastService from "primevue/toastservice";
import ConfirmationService from "primevue/confirmationservice";
import { autoAnimatePlugin } from "@formkit/auto-animate/vue";

import "./tailwind.css";
import "primeicons/primeicons.css";
import "material-symbols";

import router from "./router";

const preset = definePreset(Aura, {
  semantic: {
    primary: {
      50: "{purple.50}",
      100: "{purple.100}",
      200: "{purple.200}",
      300: "{purple.300}",
      400: "{purple.400}",
      500: "{purple.500}",
      600: "{purple.600}",
      700: "{purple.700}",
      800: "{purple.800}",
      900: "{purple.900}",
      950: "{purple.950}",
    },
  },
});

createApp(App)
  .use(PrimeVue, {
    theme: {
      preset,
      options: {
        cssLayer: {
          name: "primevue",
          order: "theme, base, primevue",
        },
      },
    },
  })
  .use(router)
  .use(ToastService)
  .use(ConfirmationService)
  .use(autoAnimatePlugin)
  .mount("#app");
