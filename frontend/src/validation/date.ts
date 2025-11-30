import * as v from 'valibot';

// Reusable validator: string → Date (ISO/Zulu timestamp expected)
export const isoDate = () =>
    v.pipe(
        v.string(),
        v.transform((value) => new Date(value)),
        v.date()
    );