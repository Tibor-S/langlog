import { cn } from "~/lib/utils";

type HangulProps = {
  hangul?: string;
} & React.ComponentProps<"div">;

export function Hangul({ hangul, className, ...props }: HangulProps) {
  const [chars, placeholder] = hangul ? [hangul, false] : ["한글", true];

  var blocks = [<Syllable syllable={chars.charAt(0)} leftmost={true} />];
  for (let i = 1; i < chars.length; i++) {
    blocks.push(<Syllable syllable={chars.charAt(i)} leftmost={false} />);
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
