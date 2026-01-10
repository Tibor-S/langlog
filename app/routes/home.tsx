import type { Route } from "./+types/home";
import { Welcome } from "../welcome/welcome";
import { useContext } from "react";
import { useOutletContext } from "react-router";
import type { AppCtx } from "~/lib/interface";
import { useAppCtx } from "~/lib/utils";
import { Hangul } from "~/components/hangul";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "New React Router App" },
    { name: "description", content: "Welcome to React Router!" },
  ];
}

export default function Home() {
  const ctx = useAppCtx();
  return (
    <div className="w-full h-full flex flex-row justify-center items-center portrait-to-col">
      <div className="bg-cyan-200 flex-2 w-full h-full"></div>
      <div className="flex-3 w-full h-full flex flex-col justify-center items-center">
        <Hangul />
      </div>
    </div>
  );
}
