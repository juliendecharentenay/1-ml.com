<template>
  <!-- Page title & actions -->
  <div class="border-b border-gray-200 px-4 py-4 sm:flex sm:items-center sm:justify-between sm:px-6 lg:px-8">
    <div class="flex-1 min-w-0">
      <h1 class="text-lg font-medium leading-6 text-gray-900 sm:truncate"> Settings </h1>
    </div>
  </div>

  <!-- Form settings -->
  <div class="mt-8">
    <div class="space-y-2 divide-y divide-gray-200 sm:space-y-3">
      <div>
        <div class="mx-4">
          <h3 class="text-lg leading-6 font-medium text-gray-900">prefix.1-ml.com</h3>
          <p class="mt-1 max-w-2xl text-sm text-gray-500">
          The prefix identifies your set of email addresses. Prefixes are case-insensitive and can only contains alphanumeric characters.<br/>
          <span class="text-sm">
          Once set, prefix can not be changed.
          </span>
          </p>
        </div>

        <div class="mx-6 mt-2 sm:mt-4 space-y-4 sm:space-y-3">
          <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start">
            <label for="prefix" class="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2"> 
              Your Prefix 
<!--
              <span class="ml-4 text-xs font-medium px-2.5 py-0.5" v-if="choose_prefix && prefix !== null">
                <span class="inline-flex items-center rounded-full bg-gray-100 text-gray-800" v-if="prefix_available === null"> Checking </span>
                <span class="inline-flex items-center rounded-full bg-green-100 text-green-800" v-if="prefix_available"> Available </span>
                <span class="inline-flex items-center rounded-full bg-pink-100 text-pink-800" v-if="prefix_available"> Not available </span>
              </span>
-->
            </label>
            <div class="mt-1 sm:mt-0 sm:col-span-2">
              <div class="flex rounded-md shadow-sm">
                <span class="inline-flex items-center px-3 rounded-l-md border border-r-0 border-gray-300 bg-gray-50 text-gray-500 sm:text-sm"> anything@ </span>
                <input type="text" name="prefix" v-model="prefix" class="flex-1 block w-full focus:ring-indigo-500 focus:border-indigo-500 min-w-0 rounded-none sm:text-sm border-gray-300" :disabled="! choose_prefix" >
                <span class="inline-flex items-center px-3 rounded-r-md border border-l-0 border-gray-300 bg-gray-50 text-gray-500 sm:text-sm"> .1-ml.com </span>
              </div>
              <p class="mt-2 text-sm text-red-600" v-if="choose_prefix && (! prefix_available)">Required action: Enter a prefix/sub-domain to identify your masked addresses using letters and numbers only and click 'Set prefix'</p>
            </div>
          </div>
        </div>

        <div class="mx-6 pt-5" v-if="choose_prefix">
          <div class="flex justify-end">
            <button type="submit" 
               class="ml-3 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white"
              :class="{'bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500': prefix_available, 'bg-indigo-300': ! prefix_available}"
              :disabled="! prefix_available"
              @click="set_prefix"
              >Set prefix</button>
          </div>
        </div>
      </div>

      <div>
        <!-- Authentication details -->
        <div class="mx-4 mt-8">
          <h3 class="text-lg leading-6 font-medium text-gray-900">Account</h3>
          <p class="mt-1 max-w-2xl text-sm text-gray-500">Your account details:</p>
        </div>

        <div class="mx-6 mt-6 sm:mt-5 space-y-6 sm:space-y-5">
          <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start">
            <label for="username" class="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2"> Username </label>
            <div class="mt-1 sm:mt-0 sm:col-span-2">
              <div class="max-w-lg flex rounded-md shadow-sm">
                <input type="text" name="username" v-model="username" id="username" class="flex-1 block w-full focus:ring-indigo-500 focus:border-indigo-500 min-w-0 rounded-md sm:text-sm border-gray-300" :disabled="true" >
              </div>
            </div>
          </div>
        </div>

        <div class="mx-6 mt-6 sm:mt-5 space-y-6 sm:space-y-5">
          <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start">
            <label for="email" class="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2"> 
              Email 
              <span class="ml-4 inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800" v-if="email_verified"> Verified </span>
              <span class="ml-4 inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800" v-if="! email_verified"> Unverified </span>
            </label>
            <div class="mt-1 sm:mt-0 sm:col-span-2">
              <div class="max-w-lg flex rounded-md shadow-sm">
                <input type="email" name="email" v-model="email" id="email" class="flex-1 block w-full focus:ring-indigo-500 focus:border-indigo-500 min-w-0 rounded-md sm:text-sm border-gray-300" :disabled="true" >
              </div>
            </div>
          </div>
        </div>

      </div>
    </div>
  </div>
</template>
<script>
import { patch } from "@/js/api.js";

export default {
  name: "SettingsView",
  emits: ["loading", "on_error", "on_user_profile_update"],
  props: ["user", "user_attributes", "user_profile"],
  data: function() {
    return {
      p_prefix: null,
    };
  },
  mounted: function() {
    window.user = this.user;
    window.user_attributes = this.user_attributes;
    this.p_prefix = this.user_profile.prefix;
  },
  computed: {
    choose_prefix: function() {
      return this.user_profile.prefix === null;
    },
    prefix_available: function() {
        return typeof this.p_prefix === 'string' && /^[a-zA-Z0-9]+$/.test(this.p_prefix);
    },
    prefix: {
      get: function() { return this.p_prefix; },
      set: function(v) { this.p_prefix = v; },
    },
    username: function() { 
      try {
        return this.user.username;
      } catch (e) {
        this.on_error("Error evaluating username", e);
      }
      return "";
    },
    email: function() {
      try {
        return this.user_attributes.email;
      } catch (e) {
        this.on_error("Error evaluating email", e);
      }
      return "";
    },
    email_verified: function() {
      try {
        return this.user_attributes.email_verified === "true";
      } catch (e) {
        this.on_error("Error evaluating email_verified", e);
      }
      return false;
    }
  },
  methods: {
    set_prefix: function() {
      try {
        this.$emit('loading', 'Check prefix availability and assign it');
        patch(this.user, '/api/me', { prefix: this.p_prefix })
        .then((p) => { this.p_prefix = p.prefix; this.$emit('on_user_profile_update', p); this.$emit('loading', null); })
        .catch((e) => { this.on_error("Error updating prefix", e); });
      } catch (e) {
        this.on_error("Error in set_prefix", e);
      }
    },
    on_error: function(message, error) {
      this.$emit('on_error', { message, error });
    }
  }
};
</script>
