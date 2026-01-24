import { cn, type ButtonProps, type SetState } from "~/lib/utils";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { useEffect, useState } from "react";
import { parsePossible, parseSyllable } from "~/lib/hangul-api";
import type { ParsedPossible, ParsedSyllable } from "~/lib/interface";
import { EJamo, jamo_from, jamo_rr } from "~/lib/interface";

type HangulProps = {
  hangul?: string;
  next?: string;
} & React.ComponentProps<"div">;

export function Hangul({ hangul, next, className, ...props }: HangulProps) {
  const [chars, placeholder] =
    hangul || next ? [hangul ? hangul : "", false] : ["한글", true];

  var blocks = [];
  for (let i = 0; i < chars.length; i++) {
    blocks.push(
      <Syllable key={i} syllable={chars.charAt(i)} leftmost={i == 0} />,
    );
  }
  if (next) {
    blocks.push(
      <Syllable key={chars.length} syllable={next} leftmost={!hangul} />,
    );
  }

  return (
    <div
      className={cn(
        className,
        "flex justify-center items-center bg-white border-2 border-gray-400 border-dashed rounded-lg text-6xl pl-2 pr-2 ",
        placeholder ? "text-gray-300" : "",
      )}
      {...props}
    >
      {blocks}
    </div>
  );
}

type HangulKeyboardProps = {
  hangul: string;
  set_hangul: SetState<string>;
  enterEntry: (hangul: string, description: string) => Promise<boolean>;
} & React.ComponentProps<"div">;

export function HangulKeyboard({
  hangul,
  set_hangul,
  enterEntry,
  className,
  ...props
}: HangulKeyboardProps) {
  const [rr, set_rr] = useState("");
  const [description, set_description] = useState("");
  const [editing, set_editing] = useState(false);
  const [possible, set_possible] = useState([] as EJamo[]);
  const handle_key = (key: EJamo | "enter" | "back") => {
    if (key == "enter") {
      set_editing(false);
    } else if (key == "back") {
      if (editing && rr.length > 1) {
        set_rr(rr.slice(0, rr.length - 1));
      } else if (editing) {
        set_hangul(hangul.slice(0, hangul.length - 1));
        set_editing(false);
      } else {
        set_hangul(hangul.slice(0, hangul.length - 1));
      }
    } else {
      set_rr(rr + jamo_rr(key));
    }
  };

  useEffect(() => {
    parsePossible(rr).then(({ jamo }: ParsedPossible) => {
      set_possible(jamo.map(jamo_from));
    });
    if (!rr) {
      return;
    }
    parseSyllable(rr).then(({ syllable, err_index }: ParsedSyllable) => {
      const syl = syllable.trim();
      if (!syl) {
        return;
      }

      if (editing) {
        var t = hangul.slice(0, hangul.length - 1);
        set_hangul(t + syl);
      } else {
        set_hangul(hangul + syl);
      }

      set_editing(true);
    });
  }, [rr]);

  useEffect(() => {
    if (!editing) {
      set_rr("");
    }
  }, [editing]);
  return (
    <div
      className={cn(className, "flex flex-col justify-between items-center")}
      {...props}
    >
      <div className="flex h-fit">
        <Hangul hangul={hangul} />
        <Button
          className="self-center ml-4 h-16 w-16 flex-col"
          onClick={(e) => {
            if (!hangul) {
              alert("Enter a valid hangul string");
            } else if (!description) {
              alert("Enter a valid description");
            } else {
              enterEntry(hangul, description).then((succ) => {
                if (succ) {
                  set_editing(false);
                  set_rr("");
                  set_hangul("");
                  set_description("");
                }
              });
            }
          }}
        >
          <img className="w-16 h-16" src="./move_item.svg" />
        </Button>
      </div>
      <Input
        placeholder="Description"
        className="mt-4"
        value={description}
        onChange={(e) => set_description(e.currentTarget.value)}
      />
      <Input
        placeholder="Hangul Input"
        className="flex-8 m-4"
        value={rr}
        onChange={(e) => {
          if (editing && e.currentTarget.value === "") {
            set_hangul(hangul.slice(0, hangul.length - 1));
            set_editing(false);
          } else {
            set_rr(e.currentTarget.value);
          }
        }}
        onKeyDown={(e) => {
          if (e.key == "Enter") {
            handle_key("enter");
          } else if (e.key == "Backspace" && e.currentTarget.value === "") {
            set_hangul(hangul.slice(0, hangul.length - 1));
          }
        }}
      />
      <div>
        <div className="ml-2.5">
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Bb}
            disabled={!possible.includes(EJamo.Bb)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Jj}
            disabled={!possible.includes(EJamo.Jj)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Dd}
            disabled={!possible.includes(EJamo.Dd)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Gg}
            disabled={!possible.includes(EJamo.Gg)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Ss}
            disabled={!possible.includes(EJamo.Ss)}
          />
          <Jamo />
          <Jamo />
          <Jamo />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Yae}
            disabled={!possible.includes(EJamo.Yae)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Ye}
            disabled={!possible.includes(EJamo.Ye)}
          />
        </div>
        <div className="ml-2.5">
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.B}
            disabled={!possible.includes(EJamo.B)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.J}
            disabled={!possible.includes(EJamo.J)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.D}
            disabled={!possible.includes(EJamo.D)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.G}
            disabled={!possible.includes(EJamo.G)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.S}
            disabled={!possible.includes(EJamo.S)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Yo}
            disabled={!possible.includes(EJamo.Yo)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Yeo}
            disabled={!possible.includes(EJamo.Yeo)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Ya}
            disabled={!possible.includes(EJamo.Ya)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Ae}
            disabled={!possible.includes(EJamo.Ae)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.E}
            disabled={!possible.includes(EJamo.E)}
          />
          <Jamo key_cb={handle_key} back />
        </div>
        <div className="ml-6">
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.M}
            disabled={!possible.includes(EJamo.M)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.N}
            disabled={!possible.includes(EJamo.N)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Ng}
            disabled={!possible.includes(EJamo.Ng)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.R}
            disabled={!possible.includes(EJamo.R)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.H}
            disabled={!possible.includes(EJamo.H)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.O}
            disabled={!possible.includes(EJamo.O)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Eo}
            disabled={!possible.includes(EJamo.Eo)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.A}
            disabled={!possible.includes(EJamo.A)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.I}
            disabled={!possible.includes(EJamo.I)}
          />
          <Jamo key_cb={handle_key} enter />
        </div>
        <div>
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.K}
            disabled={!possible.includes(EJamo.K)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.T}
            disabled={!possible.includes(EJamo.T)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Ch}
            disabled={!possible.includes(EJamo.Ch)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.P}
            disabled={!possible.includes(EJamo.P)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Yu}
            disabled={!possible.includes(EJamo.Yu)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.U}
            disabled={!possible.includes(EJamo.U)}
          />
          <Jamo
            key_cb={handle_key}
            jamo={EJamo.Eu}
            disabled={!possible.includes(EJamo.Eu)}
          />
        </div>
      </div>
    </div>
  );
}
type SyllableProps = {
  syllable: string;
  leftmost?: boolean;
  rightmost?: boolean;
} & React.ComponentProps<"div">;

export function Syllable({
  syllable,
  leftmost,
  rightmost,
  className,
  ...props
}: SyllableProps) {
  if (!syllable) {
    throw "Syllable is not a character";
  }

  const border_l = typeof leftmost == "undefined" ? false : !leftmost;

  return (
    <span
      className={cn(
        className,
        "pt-3 pb-3 text",
        border_l
          ? "ml-1 pl-1 border-l-2 border-gray-300 border-dotted"
          : undefined,
      )}
      {...props}
    >
      {syllable.charAt(0)}
    </span>
  );
}

type JamoProps = {
  jamo?: EJamo;
  enter?: boolean;
  back?: boolean;
  key_cb?: (key: EJamo | "enter" | "back") => void;
} & ButtonProps;

export function Jamo({
  jamo,
  enter,
  back,
  key_cb,
  className,
  children,
  ...props
}: JamoProps) {
  const r_jamo = jamo ? jamo : EJamo.E;
  const visible = jamo || enter || back;
  const wide = enter || back;

  var button_children;
  var cb_arg;
  if (enter || back) {
    button_children = (
      <img
        className="self-center m-0 text-lg absolute right-4 top-2"
        src={enter ? "./return.svg" : "./backspace.svg"}
      />
    );
    cb_arg = enter ? "enter" : "back";
  } else {
    button_children = (
      <>
        <span className="self-start m-0 text-lg absolute left-0 top-0">
          {String.fromCharCode(r_jamo)}
        </span>
        <span className="self-end m-0 absolute right-1 bottom-0">
          {jamo_rr(r_jamo)}
        </span>
      </>
    );
    cb_arg = r_jamo;
  }

  return (
    <Button
      className={cn(
        className,
        "relative inline-block m-0.5",
        !visible ? "hidden-2" : "",
        wide ? "w-16" : "",
      )}
      variant="outline"
      size="icon-lg"
      onClick={() =>
        !key_cb
          ? () => {}
          : enter
            ? key_cb("enter")
            : back
              ? key_cb("back")
              : key_cb(r_jamo)
      }
      {...props}
    >
      {button_children}
    </Button>
  );
}
