import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const generateRandomString = (length: number) => {
  // Define the characters that can be used in the random string.
  const characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  let result = "";
  const charactersLength = characters.length;

  // Loop 'length' times to build the string.
  for (let i = 0; i < length; i++) {
    // Get a random index from the characters string and append the character.
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
  if (!urn || typeof urn !== 'string') return "";

  if (!truncate) return urn;

  if (urn.startsWith("urn:")) {
    const parts = urn.split(":");
    if (parts.length >= 3) {
      const nid = parts[1];
      // Join the rest in case NSS contains colons, though for IDs it's usually just the last part.
      // But splitting by ':' and taking parts[2] onwards is safer.
      // However, usually urn:nid:nss. 
      // User asked for "uuid recortado a 8 char". 
      // If the NSS is "a:b:c", slicing the whole string "a:b:c" to 8 chars seems correct based on "lo que sea".
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
