<template>
  <v-app>
    <v-main>
      <v-app-bar :elevation="2">
        <template v-slot:prepend>
          <v-app-bar-nav-icon @click="drawerOpen = !drawerOpen"></v-app-bar-nav-icon>
        </template>

        <v-app-bar-title>Mainframe</v-app-bar-title>
      </v-app-bar>
      <v-navigation-drawer v-model="drawerOpen">
        <v-list-item link to="/" title="Dashboard" />
        <v-divider />
        <v-list-item link to="/recipes" title="Recipes" />
        <!-- <v-list-item link to="/shopping" title="Shopping Lists" /> -->
        <!-- <v-list-item link to="/calendar" title="Calendar" /> -->
        <!-- <v-list-item link to="/passwords" title="Passwords" /> -->
        <v-divider />
        <v-list-item link to="/administration" title="Administration" />
        <v-divider />
        <v-list-item link to="/login" title="Login" />
        <v-list-item link to="/logout" title="Logout" />
      </v-navigation-drawer>
      <router-view />
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const drawerOpen = ref(true);
</script>
