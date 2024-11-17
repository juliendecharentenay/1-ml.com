<template>
<div class="min-h-full min-w-full">
  <!-- Off-canvas menu for mobile, show/hide based on off-canvas menu state. -->
  <div class="relative z-40 lg:hidden" role="dialog" aria-modal="true" v-if="show_mobile_sidebar">
    <div class="fixed inset-0 bg-gray-600 bg-opacity-75"></div>

    <div class="fixed inset-0 flex z-40">
      <div class="relative flex-1 flex flex-col max-w-xs w-full pt-5 pb-4 bg-white">
        <div class="absolute top-0 right-0 -mr-12 pt-2">
          <button type="button" class="ml-1 flex items-center justify-center h-10 w-10 rounded-full focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
                  @click="show_mobile_sidebar = false;">
            <span class="sr-only">Close sidebar</span>
            <XMarkIcon class="h-6 w-6 text-white" />
          </button>
        </div>

        <div class="flex-shrink-0 flex items-center px-4">
          <div class="text-gray-500">Welcome to <span class="text-indigo-800">One Mail</span></div>
        </div>
        <div class="mt-5 flex-1 h-0 overflow-y-auto">
          <nav class="px-2">
            <div class="space-y-1">
              <SidebarItem :current="view === 'home'" :disable="! has_prefix()" @click="view = 'home'">
                <template #icon><HomeIcon /></template>
                Home
              </SidebarItem>
              <SidebarItem :current="view === 'settings'" :disable="user_profile === null" @click="view = 'settings'">
                <template #icon><AdjustmentsHorizontalIcon /></template>
                Settings
              </SidebarItem>
            </div>

            <div class="space-y-1 mt-2 pt-2 border-t border-gray-200">
              <SidebarItem @click="signout">
                <template #icon><ArrowRightStartOnRectangleIcon /></template>
                Logout
              </SidebarItem>
            </div>
          </nav>
        </div>
      </div>

      <div class="flex-shrink-0 w-14" aria-hidden="true">
        <!-- Dummy element to force sidebar to shrink to fit close icon -->
      </div>
    </div>
  </div>

  <!-- Static sidebar for desktop -->
  <div class="hidden lg:flex lg:flex-col lg:w-64 lg:fixed lg:inset-y-0 lg:border-r lg:border-gray-200 lg:pt-5 lg:pb-4 lg:bg-gray-100">
    <div class="flex items-center flex-shrink-0 px-6">
      <div class="text-gray-500">Welcome to <span class="text-indigo-800">One Mail</span></div>
    </div>
    <!-- Sidebar component, swap this element with another sidebar if you like -->
    <div class="mt-6 h-0 flex-1 flex flex-col overflow-y-auto">
      <!-- Sidebar Search -->
      <div class="px-3 mt-5">
        <label for="search" class="sr-only">Search</label>
        <div class="mt-1 relative rounded-md shadow-sm">
          <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none" aria-hidden="true">
            <MagnifyingGlassIcon class="mr-3 h-4 w-4 text-gray-400" />
          </div>
          <input v-model="search" type="text" name="search" id="search" class="focus:ring-indigo-500 focus:border-indigo-500 block w-full pl-9 sm:text-sm border-gray-300 rounded-md" placeholder="Search">
        </div>
      </div>
      <!-- Navigation -->
      <nav class="px-3 mt-6">
        <div class="space-y-1">
          <SidebarItem :current="view === 'home'" :disable="! has_prefix()" @click="view = 'home'">
            <template #icon><HomeIcon /></template>
            Home
          </SidebarItem>
          <SidebarItem :current="view === 'settings'" :disable="user_profile === null" @click="view = 'settings'">
            <template #icon><AdjustmentsHorizontalIcon /></template>
            Settings
          </SidebarItem>
        </div>
        <div class="space-y-1 mt-2 pt-2 border-t border-gray-200">
          <SidebarItem @click="signout">
            <template #icon><ArrowRightStartOnRectangleIcon /></template>
            Logout
          </SidebarItem>
        </div>
      </nav>
    </div>
  </div>

  <!-- Main area -->
  <div class="lg:pl-64 flex flex-col">
    <!-- Header Search Area -->
    <div class="sticky top-0 z-10 flex-shrink-0 flex h-12 bg-white border-b border-gray-200 lg:hidden">
      <!-- Sidebar toggle, controls the 'sidebarOpen' sidebar state. -->
      <button type="button" class="px-4 border-r border-gray-200 text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-purple-500 lg:hidden"
              @click="show_mobile_sidebar = true;">
        <span class="sr-only">Open sidebar</span>
        <Bars3Icon class="h-6 w-6" />
      </button>
      <div class="flex-1 flex justify-between px-4 sm:px-6 lg:px-8">
        <div class="flex-1 flex">
          <form class="w-full flex md:ml-0" action="#" method="GET">
            <label for="search-field" class="sr-only">Search</label>
            <div class="relative w-full text-gray-400 focus-within:text-gray-600">
              <div class="absolute inset-y-0 left-0 flex items-center pointer-events-none">
                <MagnifyingGlassIcon class="h-5 w-5" />
              </div>
              <input v-model="search" id="search-field" name="search-field" class="block w-full h-full pl-8 pr-3 py-2 border-transparent text-gray-900 placeholder-gray-500 focus:outline-none focus:ring-0 focus:border-transparent focus:placeholder-gray-400 text-sm" placeholder="Search" type="search">
            </div>
          </form>
        </div>
      </div>
    </div>
    <main class="flex-1">
      <VerificationView
        :user="user"
        :email="user_email"
        @loading = "loading = $event"
        @on_error = "on_error($event.message, $event.error);"
        @on_confirmed = "get_user_profile()"
        v-if="user_attributes !== null && user_verified === false" />
      <HomeView
        :user="user"
        :search="p_search.home"
        @loading = "loading = $event"
        @on_error = "on_error($event.message, $event.error);"
        v-if="user_verified && view === 'home'" />
      <SettingsView
        :user="user"
        :user_attributes="user_attributes"
        :user_profile="user_profile"
        @on_user_profile_update="update_user_profile"
        @loading = "loading = $event"
        @on_error = "on_error($event.message, $event.error);"
        v-if="user_verified && view === 'settings'" />
    </main>
  </div>
</div>

<LoadingAlert
    :message="loading"
    v-if="loading !== null" />
<ErrorAlert 
    :message="error.message"
    :error="error.error"
    @dismiss="error = null;"
    v-if="error !== null" />

</template>
<script>
import ErrorAlert from "@/components/ErrorAlert.vue";
import LoadingAlert from "@/components/LoadingAlert.vue";
import SidebarItem from "@/components/SidebarItem.vue";
import { HomeIcon, Bars3Icon, XMarkIcon, AdjustmentsHorizontalIcon } from "@heroicons/vue/24/outline";
import { ArrowRightStartOnRectangleIcon, MagnifyingGlassIcon } from "@heroicons/vue/24/solid";

import HomeView from "./components/HomeView.vue";
import SettingsView from "./components/SettingsView.vue";
import VerificationView from "./components/VerificationView.vue";

import { getCurrentCognitoUser, getUserAttributes } from "@/js/cognito.js";
import { get } from "@/js/api.js";


export default {
  name: "HomeScreen",
  components: {
    ErrorAlert,
    LoadingAlert,
    SidebarItem,
    HomeIcon,
    Bars3Icon,
    ArrowRightStartOnRectangleIcon,
    MagnifyingGlassIcon,
    XMarkIcon,
    AdjustmentsHorizontalIcon,
    HomeView,
    SettingsView,
    VerificationView,
  },
  data: function() {
    return {
      error: null,
      loading: null,
      show_mobile_sidebar: true,
      user: null,
      user_attributes: null,
      user_profile: null,
      view: 'home',
      p_search: {
        home: '',
        settings: '',
      }
    };
  },
  mounted: function() {
    try {
      // Handle case when redirection from verification of email
      if (new URL(window.location.href).searchParams.get('code') !== null) {this.show_mobile_sidebar = false;}

      window.app = this;
      this.loading = "Retrieving information";
      this.user = getCurrentCognitoUser();
      if (this.user === null) {
        this.on_error("You are not currently signed-in and will be redirected to the signing-in page shortly.", null);
        setTimeout(() => {window.location.href = "/";}, 1000);
      } else {
        getUserAttributes(this.user)
        .then((attributes) => { this.user_attributes = attributes; })
        .then(() => { this.loading = null; this.get_user_profile(); })
        .catch((e) => {this.on_error("Error whilst retrieving information", e);});
      }
    } catch (e) {
      this.on_error("Error in mounted", e);
    }
  },
  computed: {
    user_verified: function() {
      try {
        return (this.user_attributes !== null ? (this.user_attributes.email_verified === "true"): false);
      } catch (e) {
        this.on_error("Error in user_verified", e);
      }
      return false;
    },
    user_email: function() {
      try {
        return (this.user_attributes !== null ? this.user_attributes.email : undefined);
      } catch (e) {
        this.on_error("Error in user_email", e);
      }
      return "";
    },
    search: {
      get() { 
        try {
          return this.p_search[this.view]; 
        } catch (e) {
          this.on_error("Error in search::get", e);
        }
        return "";
      },
      set(v) { 
        try { 
          this.p_search[this.view] = v; 
        } catch (e) {
          this.on_error("Error in search::set", e);
        }
      },
    }
  },
  methods: {
    has_prefix: function() {
      return (this.user_profile !== null && this.user_profile.prefix !== null);
    },
    get_user_profile: function() {
      try {
        if (this.user_verified) {
          this.loading = "Retrieving user information";
          get(this.user, '/api/me')
          .then((r) => {
            this.update_user_profile(r);
            this.loading = null;
          })
          .catch((e) => {this.on_error("Error whilst retrieving user information", e);});
        }
      } catch (e) {
        this.on_error("Error in get_user_profile", e);
      }
    },
    update_user_profile: function(r) {
      try {
        this.user_profile = r;
        console.log(this.user_profile.user_id, this.user_attributes.sub);
        if (this.user_profile.user_id !== this.user_attributes.sub) { throw new Error("Your user id does not match our record..."); }
        console.log(this.user_profile.email, this.user_attributes.email);
        if (this.user_profile.email !== this.user_attributes.email) { throw new Error("Your email does not match our record..."); }
        if (! this.has_prefix()) { this.view = 'settings'; }
        
      } catch (e) {
        this.on_error("Error when updating user profile", e);
      }
    },
    signout: function() {
      try {
        this.user.signOut();
        window.location.href = "/";
      } catch (e) {
        this.on_error("Error in signout", e);
      }
    },
    on_error: function(message, error) {
      console.log(error);
      this.loading = null;
      if (this.error === null) { this.error = {message, error}; }
    }
  }
};
</script>
