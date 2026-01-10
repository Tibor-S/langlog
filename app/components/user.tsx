import type { Account } from "~/lib/interface";

type UserProps = {
  account?: Account;
} & React.ComponentProps<"a">;

export function User({ account, className, ...props }: UserProps) {
  return (
    <a className={className} {...props}>
      {account ? account.username : "Sign in"}
    </a>
  );
}
