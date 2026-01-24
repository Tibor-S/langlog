import { GalleryVerticalEnd } from "lucide-react";
import {
  data,
  redirect,
  useOutletContext,
  type ActionFunctionArgs,
  type LoaderFunctionArgs,
} from "react-router";

import { SigninForm } from "~/components/signin-form";
import { signInUp } from "~/lib/hangul-api";

export async function action({ request }: ActionFunctionArgs) {
  console.log("sign-in action");
  const form = await request.formData();
  const username = form.get("username");
  const password = form.get("password");
  console.log("username: " + username);
  console.log("password: " + password);

  if (typeof username !== "string" || typeof password !== "string") {
    return data({ error: "Invalid form data" }, { status: 400 });
  }

  const account = await signInUp({
    username: username,
    password: password,
  }).then((account) => {
    console.log(account);
    return account;
  });

  if (!account) {
    return data({ error: "Could not sign in or signup" }, { status: 500 });
  }

  return redirect("/", {
    headers: {
      "Set-Cookie": "account=" + JSON.stringify(account),
    },
  });
}

export default function Signin() {
  return (
    <div className="w-full h-full flex justify-center items-center">
      <SigninForm />
    </div>
  );
}
