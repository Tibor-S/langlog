import { cn, setOnChange, type SetState } from "~/lib/utils";
import { Button } from "~/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";
import {
  Field,
  FieldDescription,
  FieldGroup,
  FieldLabel,
  FieldSeparator,
} from "~/components/ui/field";
import { Input } from "~/components/ui/input";
import {
  useEffect,
  useState,
  type ChangeEvent,
  type Dispatch,
  type FormEvent,
  type SetStateAction,
} from "react";
import type { Account } from "~/lib/interface";
import { redirect, type ActionFunctionArgs } from "react-router";
import { signInUp, signUp } from "~/lib/hangul-api";
import { data } from "react-router";

export function SigninForm({
  className,
  ...props
}: React.ComponentProps<"div">) {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  return (
    <div
      className={cn("flex flex-col gap-6 max-w-3xl w-full min-w-md", className)}
      {...props}
    >
      <Card>
        <CardHeader className="text-center">
          <CardTitle className="text-xl">Sign in / Sign up</CardTitle>
        </CardHeader>
        <CardContent>
          <form method="POST">
            <FieldGroup>
              <Field>
                <FieldLabel htmlFor="username">Username</FieldLabel>
                <Input
                  name="username"
                  id="username"
                  type="text"
                  required
                  // onChange={setOnChange(setUsername)}
                />
              </Field>
              <Field>
                <FieldLabel htmlFor="password">Password</FieldLabel>
                <Input
                  name="password"
                  id="password"
                  type="password"
                  required
                  // onChange={setOnChange(setPassword)}
                />
              </Field>
              <Field>
                <Button type="submit">Sign in</Button>
              </Field>
            </FieldGroup>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}
