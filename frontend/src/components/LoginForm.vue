<template>
    <Form @submit="onSubmit" :validation-schema="schema" :initial-values="initialValues">
        <Field name="username" v-slot="{ field, errorMessage }">
            <v-text-field v-bind="field" :error-messages="errorMessage ? [errorMessage] : []" clearable label="Username"
                variant="outlined" class="mb-3" />
        </Field>

        <Field name="password" v-slot="{ field, errorMessage }">
            <v-text-field v-bind="field" :error-messages="errorMessage ? [errorMessage] : []" clearable label="Password"
                type="password" variant="outlined" class="mb-4" />
        </Field>

        <v-btn type="submit" color="primary" size="large" block>
            Login
        </v-btn>
    </Form>
</template>

<script setup lang="ts">
import { Form, Field, type GenericObject } from 'vee-validate';
import { LoginRequestSchema, useUserStore, type LoginRequest } from '@/stores/user';
import { toTypedSchema } from '@vee-validate/valibot';
import * as v from 'valibot';
import { useRouter } from 'vue-router';

const router = useRouter();
const userStore = useUserStore();
const initialValues = {
    username: '',
    password: ''
}

const schema = toTypedSchema(LoginRequestSchema);

async function onSubmit(values: GenericObject) {
    console.log('Form values:', values);
    const request: LoginRequest = v.parse(LoginRequestSchema, values);
    await userStore.login(request);
    await router.replace({ name: 'Dashboard' });
}
</script>