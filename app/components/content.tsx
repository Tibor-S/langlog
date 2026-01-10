import { cn } from "~/lib/utils";

export function Content({
  className,
  children,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div className={cn("flex flex-row justify-center", className)} {...props}>
      <div className="flex flex-row justify-center w-full min-w-sm max-w-7xl pt-2">
        {children}
      </div>
    </div>
  );
}
