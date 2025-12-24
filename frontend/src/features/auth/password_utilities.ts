export const ALLOWED_SPECIALS = "!@#$%^&*()_+-=[]{}";

export const generateRandomPassword = (length: number = 12): string => {
  if (length < 8) {
    throw new Error("Password length must be at least 8 to meet requirements");
  }

  const upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  const lower = "abcdefghijklmnopqrstuvwxyz";
  const digits = "0123456789";
  const specials = ALLOWED_SPECIALS;

  const getRandomChar = (charset: string) => {
    const randomValues = new Uint32Array(1);
    crypto.getRandomValues(randomValues);
    return charset[randomValues[0] % charset.length];
  };

  // Guarantee minimums
  const required = [
    getRandomChar(upper),
    getRandomChar(upper),
    getRandomChar(lower),
    getRandomChar(lower),
    getRandomChar(digits),
    getRandomChar(digits),
    getRandomChar(specials),
    getRandomChar(specials),
  ];

  // Fill remaining length with random characters from all sets
  const allChars = upper + lower + digits + specials;
  const remaining = length - required.length;

  for (let i = 0; i < remaining; i++) {
    required.push(getRandomChar(allChars));
  }

  // Shuffle the array using Fisher-Yates algorithm
  for (let i = required.length - 1; i > 0; i--) {
    const randomValues = new Uint32Array(1);
    crypto.getRandomValues(randomValues);
    const j = randomValues[0] % (i + 1);
    [required[i], required[j]] = [required[j], required[i]];
  }

  return required.join("");
};
