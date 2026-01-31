import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const generateRandomString = (length: number) => {
  const characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  let result = "";
  const charactersLength = characters.length;
  for (let i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
};

export const mergeStateAndAttribute = (state: string, attribute: string): string => {
  let textOut = "";
  switch (state) {
    case "SUSPENDED":
      switch (attribute) {
        case "BY_PROVIDER":
        case "BY_CONSUMER":
          textOut = `${state} ${attribute.replace("_", " ")}`;
          break;
        default:
          textOut = state;
      }
      break;
    default:
      textOut = state;
  }
  return textOut;
};

export const formatUrn = (urn: string | undefined, truncate: boolean = true): string => {
  if (!urn || typeof urn !== "string") return "";

  if (!truncate) return urn;

  if (urn.startsWith("urn:")) {
    const parts = urn.split(":");
    if (parts.length >= 3) {
      const nid = parts[1];
      const nss = parts.slice(2).join(":");
      const shortNid = nid.length > 7 ? nid.slice(0, 7) : nid;
      const shortNss = nss.length > 8 ? nss.slice(0, 8) : nss;

      return `urn:${shortNid}:${shortNss}`;
    }
  }

  // Fallback for non-URN strings (preserve old behavior or just standard truncate)
  if (urn.length > 20) {
    return urn.slice(0, 13) + "[...]";
  }

  return urn;
};
