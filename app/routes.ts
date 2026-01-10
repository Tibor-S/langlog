import {
  type RouteConfig,
  type RouteConfigEntry,
  index,
} from "@react-router/dev/routes";

export default [
  index("routes/home.tsx"),
  {
    path: "/signin",
    file: "routes/signin.tsx",
  } satisfies RouteConfigEntry,
] satisfies RouteConfig;
