import { createRouter, createWebHashHistory } from "vue-router";
import Login from "./Login.vue";
import Home from "./Home.vue";
import Register from "./Register.vue";

const routes = [
  { path: "/", name: "Start", component: Home },
  { path: "/login", name: "Login", component: Login },
  { path: "/register", name: "Register", component: Register },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;