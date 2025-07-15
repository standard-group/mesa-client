import { createApp, h, Suspense } from "vue";
import App from "./App.vue";
import router from "./router";

createApp({
  render: () => h(Suspense, null, {
    default: () => h(App),
    fallback: () => h("div", "Loading...")
  })
})
.use(router)
.mount("#app");
