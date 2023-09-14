import { css } from "@/panda";

export default function Btn({ children, ...props }) {
  return (
    <>
      <button
        className={css({
          _hover: { bg: "blue.100" },
          cursor: "pointer",
          borderRadius: "10px",
          padding: "0.5em",
        })}
        {...props}
      >
        {children}
      </button>
    </>
  );
}
