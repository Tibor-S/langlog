import { clsx, type ClassValue } from "clsx";
import type { ChangeEvent, Dispatch, SetStateAction } from "react";
import React from "react";
import { twMerge } from "tailwind-merge";
import type { Account, AppCtx, Cookies } from "./interface";
import { useOutletContext } from "react-router";

export type SetState<T> = Dispatch<SetStateAction<T>>;

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function setOnChange(set: SetState<string>) {
  return (e: ChangeEvent<HTMLInputElement>) => {
    set(e.target.value);
  };
}
export function parseOnChange<T>(
  set: SetState<T>,
  parse: (value: string) => T,
) {
  return (e: ChangeEvent<HTMLInputElement>) => {
    set(parse(e.target.value));
  };
}

export function parseCookies(cookieString: string): Cookies {
  var cookies = {} as Cookies;

  for (const cookie of cookieString.split(";")) {
    const [name, value] = cookie.split("=");
    switch (name) {
      case "account":
        cookies.account = JSON.parse(value);
    }
  }

  return cookies;
}

export function useAppCtx(): AppCtx {
  return useOutletContext<AppCtx>();
}
