import { createMemoryHistory, createRouter } from "vue-router";

import ProjectView from "./routes/ProjectView.vue";
import TracksView from "./routes/TracksView.vue";
import PatternsView from "./routes/PatternsView.vue";

const routes = [
  { path: "/", component: ProjectView },
  { path: "/tracks", component: TracksView },
  { path: "/patterns", component: PatternsView },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
