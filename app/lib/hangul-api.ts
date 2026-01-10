import type { Account, SignIn } from "./interface";

export async function signInUp(signin: SignIn): Promise<Account | undefined> {
  let opt = await signIn(signin);
  return opt ? opt : await signUp(signin);
}

export async function signIn(signin: SignIn): Promise<Account | undefined> {
  return fetch("https://hangul-api.tibors.se/signin", {
    method: "POST",
    body: JSON.stringify(signin),
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json;charset=UTF-8",
    },
  })
    .then((res) => res.json())
    .then((j) => j as Account | undefined);
}
export async function signUp(signin: SignIn): Promise<Account | undefined> {
  return fetch("https://hangul-api.tibors.se/signin", {
    method: "POST",
    body: JSON.stringify(signin),
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json;charset=UTF-8",
    },
  })
    .then((res) => res.json())
    .then((j) => j as Account | undefined);
}
