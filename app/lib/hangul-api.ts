import type {
  Account,
  LogEntry,
  ParsedHangul,
  ParsedPossible,
  ParsedSyllable,
  SignIn,
} from "./interface";

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

export async function parseSyllable(text?: string): Promise<ParsedSyllable> {
  return fetch(
    "https://hangul-api.tibors.se/parse/syllable?text=" + (text ? text : ""),
  )
    .then((res) => {
      return res.json();
    })
    .then((j) => {
      return j as ParsedSyllable;
    });
}

export async function parsePossible(text?: string): Promise<ParsedPossible> {
  return fetch(
    "https://hangul-api.tibors.se/parse/possible?text=" + (text ? text : ""),
  )
    .then((res) => {
      return res.json();
    })
    .then((j) => {
      return j as ParsedPossible;
    });
}

export async function insertLog(
  account: Account,
  hangul: string,
  description: string,
): Promise<string | undefined> {
  return fetch("https://hangul-api.tibors.se/log/insert", {
    method: "POST",
    body: JSON.stringify({
      account: account,
      hangul: hangul,
      description: description,
    }),
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json;charset=UTF-8",
    },
  }).then((res) => {
    return res.status != 205 ? "Could not submit entry" : undefined;
  });
}

export async function getLog(account: Account): Promise<LogEntry[]> {
  return fetch("https://hangul-api.tibors.se/log", {
    method: "POST",
    body: JSON.stringify(account),
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json;charset=UTF-8",
    },
  })
    .then((res) => {
      return res.json();
    })
    .then((j) => {
      return j as LogEntry[];
    });
}

export async function deleteLog(
  account: Account,
  hangul: string,
): Promise<string | undefined> {
  return fetch("https://hangul-api.tibors.se/log/delete", {
    method: "DELETE",
    body: JSON.stringify({
      account: account,
      hangul: hangul,
    }),
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json;charset=UTF-8",
    },
  }).then((res) => {
    return res.status != 205 ? "Could not delete entry" : undefined;
  });
}
