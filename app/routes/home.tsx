import type { Route } from "./+types/home";
import { useContext, useEffect, useState, type ReactElement } from "react";
import { useOutletContext } from "react-router";
import type { AppCtx, LogEntry } from "~/lib/interface";
import { cn, useAppCtx } from "~/lib/utils";
import { Hangul, HangulKeyboard } from "~/components/hangul";
import { deleteLog, getLog, insertLog } from "~/lib/hangul-api";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
} from "~/components/ui/card";
import { Button } from "~/components/ui/button";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "New React Router App" },
    { name: "description", content: "Welcome to React Router!" },
  ];
}

export default function Home() {
  const ctx = useAppCtx();
  const [hangul, set_hangul] = useState("");
  const [log, set_log] = useState([] as ReactElement[]);
  const reloadLog = () => {
    getLog(ctx.account)
      .then((logs) => {
        var logs2 = [];
        for (const { hangul_log_id, hangul, description } of logs) {
          logs2.push(
            <div key={hangul_log_id} className={cn("m-4 max-w-100 w-full")}>
              <Card>
                <CardHeader className="flex justify-between">
                  <span className="text-2xl">{hangul}</span>
                  <Button
                    className="w-9 h-9 p-2"
                    onClick={(e) => {
                      deleteLog(ctx.account, hangul).then((err) => {
                        if (err) {
                          alert(err);
                        } else {
                          reloadLog();
                        }
                      });
                    }}
                  >
                    <img src="./delete.svg" />
                  </Button>
                </CardHeader>
                <CardContent className="text-xl">{description}</CardContent>
              </Card>
            </div>,
          );
        }
        if (logs.length % 2 == 1) {
          logs2.push(
            <div key={-1} className={cn("m-4 max-w-100 w-full hidden-2")}>
              <Card>
                <CardHeader className="text-2xl">네</CardHeader>
                <CardContent className="text-xl">hidden</CardContent>
              </Card>
            </div>,
          );
        }
        set_log(logs2);
      })
      .catch(() => {
        set_log([]);
      });
  };

  if (log.length == 0) {
    reloadLog();
  }

  return (
    <>
      <div className="w-full min-h-[70dvh] flex flex-col justify-center items-center">
        <HangulKeyboard
          hangul={hangul}
          set_hangul={set_hangul}
          enterEntry={(hangul: string, description: string) =>
            insertLog(ctx.account, hangul, description).then((msg) => {
              if (msg) {
                alert(msg);
                return false;
              } else {
                reloadLog();
                return true;
              }
            })
          }
        />
      </div>

      {/*<h2 className="text-2xl ml-32">Log</h2>*/}
      <div className="w-full flex flex-row flex-wrap justify-around">{log}</div>
    </>
  );
}
