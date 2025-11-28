import { defineStore } from 'pinia'

import * as v from 'valibot';
import { useToast } from 'vue-toastification';

const UserSchema = v.object({
    id: v.number(),
    firstName: v.string(),
    lastName: v.string(),
    email: v.string(),
    username: v.string(),
    lastLogin: v.optional(v.date()),
    isAdmin: v.boolean(),
});

type User = v.InferOutput<typeof UserSchema>;

export const LoginRequestSchema = v.object({
    username: v.pipe(v.string('Please enter your username'), v.minLength(1, 'Username is required')),
    password: v.pipe(v.string('Please enter your password'), v.minLength(1, 'Password is required')),
});

export type LoginRequest = v.InferOutput<typeof LoginRequestSchema>;

export const useUserStore = defineStore('user', {
    state: () => ({
        currentUser: null as User | null,
        isLoggedIn: false,
    }),

    getters: {
        userName: (state) => state.currentUser?.firstName,
        userFullName: (state) => [state.currentUser?.lastName, state.currentUser?.firstName].join(', '),
    },

    actions: {
        async login(request: LoginRequest) {
            const toast = useToast();
            try {
                const response = await fetch('/api/users/login', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(request)
                });

                if (!response.ok) {
                    if (response.status === 401) {
                        toast.error('Invalid username or password');
                        return;
                    }

                    throw new Error('Http response was not success');
                }

                const data = await response.json();
                const parseResult = v.safeParse(UserSchema, data);

                if (parseResult.success) {
                    this.currentUser = parseResult.output;
                    this.isLoggedIn = true;
                    toast.success('Welcome back!');
                    return;
                }

                throw new Error('Server response data was not the correct type');
            } catch (error) {
                console.error('Login failed:', error)
                toast.error('Server error. Please try again later.', { timeout: false });
                this.isLoggedIn = false
                this.currentUser = null
            }
        },

        logout() {
            // TODO: call the backend logout function to get rid of the current session.
            this.currentUser = null
            this.isLoggedIn = false
        }
    }
})

