import { defineStore } from 'pinia'

export const useUserStore = defineStore('user', {
    state: () => ({
        currentUser: null as User | null,
        isLoggedIn: false,
        token: ''
    }),

    getters: {
        userName: (state) => state.currentUser?.name ?? 'Guest'
    },

    actions: {
        async login(username: string, password: string) {
            try {
                const response = await fetch('/api/auth/login', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ username, password })
                })

                const data = await response.json()

                this.currentUser = data.user
                this.token = data.token
                this.isLoggedIn = true
            } catch (error) {
                console.error('Login failed:', error)
                throw error
            }
        },

        logout() {
            this.currentUser = null
            this.token = ''
            this.isLoggedIn = false
        }
    }
})

interface User {
    id: number
    name: string
    email: string
}