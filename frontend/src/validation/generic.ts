import * as v from 'valibot';

// Create a factory function that takes the item schema
export function paginatedResponseSchema<T extends v.BaseSchema<any, any, any>>(
    itemSchema: T
) {
    return v.object({
        data: v.array(itemSchema),
        page: v.pipe(v.number(), v.integer()),
        pageSize: v.pipe(v.number(), v.integer()),
        total: v.pipe(v.number(), v.integer()),
        totalPages: v.pipe(v.number(), v.integer()),
    });
}