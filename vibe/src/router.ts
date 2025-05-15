import { createMemoryHistory, createRouter, RouteRecordRaw } from "vue-router";

import ProjectView from "./routes/ProjectView.vue";
import TracksView from "./routes/TracksView.vue";
import PatternsView from "./routes/PatternsView.vue";
import ControlsView from "./routes/ControlsView.vue";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "Project",
    meta: { icon: "settings" },
    component: ProjectView,
  },
  {
    path: "/tracks",
    name: "Tracks",
    meta: { icon: "queue_music" },
    component: TracksView,
  },
  {
    path: "/patterns",
    name: "Patterns",
    meta: { icon: "library_music" },
    component: PatternsView,
  },
  {
    path: "/controls",
    name: "Controls",
    meta: { icon: "instant_mix" },
    component: ControlsView,
  },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
