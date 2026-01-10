import type { Dispatch, SetStateAction } from "react";

export interface AppCtx {
  account: Account;
}

export interface Cookies {
  account?: Account;
}

export interface Account {
  account_id: number;
  username: string;
}

export interface SignIn {
  username: string;
  password: string;
}
