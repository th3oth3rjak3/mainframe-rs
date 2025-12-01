import { defineStore } from 'pinia'
import { isoDate } from '@/validation/date';

import * as v from 'valibot';
import { useToast } from 'vue-toastification';
import { useRouter } from 'vue-router';

const UserSchema = v.object({
    id: v.number(),
    firstName: v.string(),
    lastName: v.string(),
    email: v.string(),
    username: v.string(),
    lastLogin: v.nullable(isoDate()),
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
    }),

    getters: {
        userName: (state) => state.currentUser?.firstName,
        userFullName: (state) => {
            if (!state.currentUser) return '';
            return `${state.currentUser.lastName}, ${state.currentUser.firstName}`;
        },
        isLoggedIn: (state) => state.currentUser !== null,
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
                    toast.success('Welcome back!');
                    return;
                }

                throw new Error('Server response data was not the correct type');
            } catch (error) {
                console.error('Login failed:', error)
                toast.error('Server error. Please try again later.', { timeout: false });
                this.currentUser = null
            }
        },
        async hydrateUser() {
            try {
                const response = await fetch('/api/users/self', {
                    method: 'GET',
                    credentials: 'include',
                });

                if (!response.ok) {
                    this.currentUser = null;
                    return;
                }

                const data = await response.json();
                const parseResult = v.safeParse(UserSchema, data);

                if (parseResult.success) {
                    this.currentUser = parseResult.output;
                } else {
                    console.error('Invalid user data from server');
                    this.currentUser = null;
                }
            } catch (err) {
                console.error('Failed to hydrate user:', err);
                this.currentUser = null;
            }
        },

        async logout() {
            const toast = useToast();
            try {
                const response = await fetch('/api/users/logout', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: null,
                });

                if (!response.ok) {
                    toast.error("Logout failed on the server");
                    throw new Error('Http response was not success');
                }

                this.currentUser = null;
            } catch (error) {
                console.error('Logout failed:', error)
                toast.error('Server error. Please try again later.', { timeout: false });
            }
        }
    }
})

