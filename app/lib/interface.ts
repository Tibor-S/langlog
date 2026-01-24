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

export interface ParsedHangul {
  hangul: string;
  err_index?: number;
}

export interface ParsedSyllable {
  syllable: string;
  err_index?: number;
}

export interface ParsedPossible {
  jamo: string[];
  err_index?: number;
}

export interface LogEntry {
  hangul_log_id: number;
  hangul: string;
  description: string;
}

export enum EJamo {
  ///ㄱ
  G = 0x3131,
  ///ㄲ
  Gg,
  ///ㄳ
  Gs,
  ///ㄴ
  N,
  ///ㄵ
  Nc,
  ///ㄶ
  Nch,
  ///ㄷ
  D,
  ///ㄸ
  Dd,
  ///ㄹ
  R,
  ///ㄺ
  Lg,
  ///ㄻ
  Lm,
  ///ㄼ
  Lb,
  ///ㄽ
  Ls,
  ///ㄾ
  Lt,
  ///ㄿ
  Lph,
  ///ㅀ
  Lh,
  ///ㅁ
  M,
  ///ㅂ
  B,
  ///ㅃ
  Bb,
  ///ㅄ
  Bs,
  ///ㅅ
  S,
  ///ㅆ
  Ss,
  ///ㅇ
  Ng,
  ///ㅈ
  J,
  ///ㅉ
  Jj,
  ///ㅊ
  Ch,
  ///ㅋ
  K,
  ///ㅌ
  T,
  ///ㅍ
  P,
  ///ㅎ
  H,
  ///ㅏ
  A,
  ///ㅐ
  Ae,
  ///ㅑ
  Ya,
  ///ㅒ
  Yae,
  ///ㅓ
  Eo,
  ///ㅔ
  E,
  ///ㅕ
  Yeo,
  ///ㅖ
  Ye,
  ///ㅗ
  O,
  ///ㅘ
  Wa,
  ///ㅙ
  Wae,
  ///ㅚ
  Oe,
  ///ㅛ
  Yo,
  ///ㅜ
  U,
  ///ㅝ
  Wo,
  ///ㅞ
  We,
  ///ㅟ
  Wi,
  ///ㅠ
  Yu,
  ///ㅡ
  Eu,
  ///ㅢ
  Ui,
  ///ㅣ
  I,
}

export function jamo_rr(jamo: EJamo) {
  switch (jamo) {
    case EJamo.G:
      return "G".toLowerCase();
    case EJamo.Gg:
      return "Gg".toLowerCase();
    case EJamo.Gs:
      return "Gs".toLowerCase();
    case EJamo.N:
      return "N".toLowerCase();
    case EJamo.Nc:
      return "Nc".toLowerCase();
    case EJamo.Nch:
      return "Nch".toLowerCase();
    case EJamo.D:
      return "D".toLowerCase();
    case EJamo.Dd:
      return "Dd".toLowerCase();
    case EJamo.R:
      return "R".toLowerCase();
    case EJamo.Lg:
      return "Lg".toLowerCase();
    case EJamo.Lm:
      return "Lm".toLowerCase();
    case EJamo.Lb:
      return "Lb".toLowerCase();
    case EJamo.Ls:
      return "Ls".toLowerCase();
    case EJamo.Lt:
      return "Lt".toLowerCase();
    case EJamo.Lph:
      return "Lph".toLowerCase();
    case EJamo.Lh:
      return "Lh".toLowerCase();
    case EJamo.M:
      return "M".toLowerCase();
    case EJamo.B:
      return "B".toLowerCase();
    case EJamo.Bb:
      return "Bb".toLowerCase();
    case EJamo.Bs:
      return "Bs".toLowerCase();
    case EJamo.S:
      return "S".toLowerCase();
    case EJamo.Ss:
      return "Ss".toLowerCase();
    case EJamo.Ng:
      return "Ng".toLowerCase();
    case EJamo.J:
      return "J".toLowerCase();
    case EJamo.Jj:
      return "Jj".toLowerCase();
    case EJamo.Ch:
      return "Ch".toLowerCase();
    case EJamo.K:
      return "K".toLowerCase();
    case EJamo.T:
      return "T".toLowerCase();
    case EJamo.P:
      return "P".toLowerCase();
    case EJamo.H:
      return "H".toLowerCase();
    case EJamo.A:
      return "A".toLowerCase();
    case EJamo.Ae:
      return "Ae".toLowerCase();
    case EJamo.Ya:
      return "Ya".toLowerCase();
    case EJamo.Yae:
      return "Yae".toLowerCase();
    case EJamo.Eo:
      return "Eo".toLowerCase();
    case EJamo.E:
      return "E".toLowerCase();
    case EJamo.Yeo:
      return "Yeo".toLowerCase();
    case EJamo.Ye:
      return "Ye".toLowerCase();
    case EJamo.O:
      return "O".toLowerCase();
    case EJamo.Wa:
      return "Wa".toLowerCase();
    case EJamo.Wae:
      return "Wae".toLowerCase();
    case EJamo.Oe:
      return "Oe".toLowerCase();
    case EJamo.Yo:
      return "Yo".toLowerCase();
    case EJamo.U:
      return "U".toLowerCase();
    case EJamo.Wo:
      return "Wo".toLowerCase();
    case EJamo.We:
      return "We".toLowerCase();
    case EJamo.Wi:
      return "Wi".toLowerCase();
    case EJamo.Yu:
      return "Yu".toLowerCase();
    case EJamo.Eu:
      return "Eu".toLowerCase();
    case EJamo.Ui:
      return "Ui".toLowerCase();
    case EJamo.I:
      return "I".toLowerCase();
  }
}

export function jamo_from(jamo: string) {
  switch (jamo) {
    case "ㄱ":
      return EJamo.G;
    case "ㄲ":
      return EJamo.Gg;
    case "ㄳ":
      return EJamo.Gs;
    case "ㄴ":
      return EJamo.N;
    case "ㄵ":
      return EJamo.Nc;
    case "ㄶ":
      return EJamo.Nch;
    case "ㄷ":
      return EJamo.D;
    case "ㄸ":
      return EJamo.Dd;
    case "ㄹ":
      return EJamo.R;
    case "ㄺ":
      return EJamo.Lg;
    case "ㄻ":
      return EJamo.Lm;
    case "ㄼ":
      return EJamo.Lb;
    case "ㄽ":
      return EJamo.Ls;
    case "ㄾ":
      return EJamo.Lt;
    case "ㄿ":
      return EJamo.Lph;
    case "ㅀ":
      return EJamo.Lh;
    case "ㅁ":
      return EJamo.M;
    case "ㅂ":
      return EJamo.B;
    case "ㅃ":
      return EJamo.Bb;
    case "ㅄ":
      return EJamo.Bs;
    case "ㅅ":
      return EJamo.S;
    case "ㅆ":
      return EJamo.Ss;
    case "ㅇ":
      return EJamo.Ng;
    case "ㅈ":
      return EJamo.J;
    case "ㅉ":
      return EJamo.Jj;
    case "ㅊ":
      return EJamo.Ch;
    case "ㅋ":
      return EJamo.K;
    case "ㅌ":
      return EJamo.T;
    case "ㅍ":
      return EJamo.P;
    case "ㅎ":
      return EJamo.H;
    case "ㅏ":
      return EJamo.A;
    case "ㅐ":
      return EJamo.Ae;
    case "ㅑ":
      return EJamo.Ya;
    case "ㅒ":
      return EJamo.Yae;
    case "ㅓ":
      return EJamo.Eo;
    case "ㅔ":
      return EJamo.E;
    case "ㅕ":
      return EJamo.Yeo;
    case "ㅖ":
      return EJamo.Ye;
    case "ㅗ":
      return EJamo.O;
    case "ㅘ":
      return EJamo.Wa;
    case "ㅙ":
      return EJamo.Wae;
    case "ㅚ":
      return EJamo.Oe;
    case "ㅛ":
      return EJamo.Yo;
    case "ㅜ":
      return EJamo.U;
    case "ㅝ":
      return EJamo.Wo;
    case "ㅞ":
      return EJamo.We;
    case "ㅟ":
      return EJamo.Wi;
    case "ㅠ":
      return EJamo.Yu;
    case "ㅡ":
      return EJamo.Eu;
    case "ㅢ":
      return EJamo.Ui;
    case "ㅣ":
      return EJamo.I;
  }
  return EJamo.A;
}
