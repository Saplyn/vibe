import { createMemoryHistory, createRouter, RouteRecordRaw } from "vue-router";

import ProjectView from "./routes/ProjectView.vue";
import TracksView from "./routes/TracksView.vue";
import PatternsView from "./routes/PatternsView.vue";
import SlidersView from "./routes/SlidersView.vue";
import EventsView from "./routes/EventsView.vue";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "Project",
    meta: { icon: "settings", edit: false },
    component: ProjectView,
  },
  {
    path: "/tracks",
    name: "Tracks",
    meta: { icon: "queue_music", edit: false },
    component: TracksView,
  },
  {
    path: "/patterns",
    name: "Patterns",
    meta: { icon: "library_music", edit: true },
    component: PatternsView,
  },
  {
    path: "/sliders",
    name: "Sliders",
    meta: { icon: "instant_mix", edit: false },
    component: SlidersView,
  },
  {
    path: "/events",
    name: "Events",
    meta: { icon: "auto_read_play", edit: false },
    component: EventsView,
  },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
