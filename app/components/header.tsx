import { cn } from "~/lib/utils";
import { Logo } from "./logo";
import { User } from "./user";
import type { Account } from "~/lib/interface";

type HeaderProps = {
  account?: Account;
} & React.ComponentProps<"div">;

export function Header({ account, className, ...props }: HeaderProps) {
  return (
    <div
      className={cn(
        "h-12 pl-2 pr-2 flex items-center justify-between flex-row",
        className,
      )}
      {...props}
    >
      <Logo />
      <User account={account} />
    </div>
  );
}
