<template>
      <!-- Page title & actions -->
      <div class="hidden sm:block border-b border-gray-200 px-4 py-2 sm:flex sm:items-center sm:justify-between sm:px-6 lg:px-8">
        <div class="flex-1 min-w-0">
          <h1 class="text-lg font-medium leading-6 text-gray-900 sm:truncate"> Home </h1>
        </div>
      </div>

      <!-- No email message -->
      <div class="mt-8" v-if="emails.length === 0">
        <p class="px-6 text-gray-500">
        No emails have been used on your account. Add something to explain how it works...
        </p>
      </div>

      <!-- table (small breakpoint and up) -->
      <div class="mt-4" v-if="emails.length > 0">
        <div class="align-middle inline-block min-w-full border-b border-gray-200">
          <table class="min-w-full">
            <thead>
              <tr class="border-t border-gray-200">
                <th class="px-6 py-3 border-b border-gray-200 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider" scope="col">
                  <span class="lg:pl-2">Address</span>
                </th>
                <th class="hidden md:table-cell px-6 py-3 border-b border-gray-200 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider text-center" scope="col">
                  <div class="md:block lg:hidden xl:block">
                    <div>Last activity</div><div># recent emails/total emails</div>
                  </div>
                  <div class="hidden lg:block xl:hidden">
                    <div># recent emails</div><div>total emails</div>
                  </div>
                </th>
                <th class="pr-6 md:pr-12 lg:pr-24 py-3 border-b border-gray-200 bg-gray-50 text-right text-xs font-medium text-gray-500 uppercase tracking-wider" scope="col">
                  Status
                </th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-gray-100 ">
              <tr v-for="email in emails"
                  :key="email.email"
                  >
                <td class="px-2 sm:px-4 md:px-6 py-1 md:py-2 text-sm font-medium text-gray-900">
                  <div class="lg:pl-2 truncate">
                    {{ email.email }}
                  </div>
                </td>
                <td class="hidden md:table-cell px-6 py-3 text-sm text-gray-500 font-medium text-center">
                  <div v-if="email.count_all_time > 0">
                    <div class="hidden md:block lg:hidden xl:block">
                      <div v-if="email.last_email !== null">
                        {{ email.last_email.split(" ")[0] }}
                      </div>
                      <div v-else>-</div>
                      <div>Recent: {{ email.count_6_days }}/All time: {{ email.count_all_time }}</div>
                    </div>
                    <div class="hidden lg:block xl:hidden">
                      <div>Recent: {{ email.count_6_days }}</div>
                      <div>All time: {{ email.count_all_time }}</div>
                    </div>
                  </div>
                  <div v-else>-</div>
                </td>
                <td class="pr-6 md:pr-12 lg:pr-24 py-1 md:py-2 whitespace-nowrap text-sm font-medium flex flex-row justify-end">
                  <div>
                    <select class="mt-1 block pl-3 pr-10 py-1 text-sm sm-gray-300 ring-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
                            @change="set_status(email.email, $event.target.value)"
                            >
                      <option value="Forward" :selected="email.status === 'Forward'"
                              >Forward</option>
                      <option value="ForwardAsText" :selected="email.status === 'ForwardAsText'"
                              >Forward as text</option>
                      <option value="Block" :selected="email.status === 'Block'"
                              >Block</option>
                    </select>
                  </div>
                </td>
              </tr>

              <!-- More projects... -->
            </tbody>
          </table>
        </div>
      </div>
</template>
<script>
import { get, patch } from "@/js/api.js";

export default {
  name: "HomeView",
  props: ['user', 'search'],
  emits: ["loading", "on_error"],
  components: {
  },
  data: function() {
    return {
      p_emails: [],
    };
  },
  mounted: function() {
    try {
      get(this.user, "/api/email")
      .then((r) => {this.p_emails = r;})
      .catch((e) => {this.on_error("Error when querying list of emails", e);});
    } catch (e) {
      this.on_error("Error in mounted", e);
    }
  },
  computed: {
    emails: function() {
      try {
        let emails = this.p_emails;
        if (this.search !== null && this.search !== "") {
          let re = new RegExp(this.search, "i");
          emails = emails.filter((e) => re.test(e.email));
        }
        return emails.sort((a, b) => {
          if (a.last_email === b.last_email) {
            if (a.count_6_days !== b.count_6_days) {
              return b.count_6_days - a.count_6_days;
            } else {
              return b.count_all_time - a.count_all_time;
            }
          } else if (a.last_email !== null) {
            if (b.last_email !== null) {
              if (a.last_email > b.last_email) {
                return -1;
              } else { // As a.last_email !== b.last_email
                return 1;
              }
            } else {
              return -1;
            }
          } else { // if (b.last_email !== null) {
            return 1;
          }
        });
      } catch(e) {
        this.on_error("Error in emails", e);
      }
      return [];
    }
  },
  methods: {
    set_status: function(email, status) {
      try {
        console.log("set_status", email, status);
        patch(this.user, `/api/email/${encodeURIComponent(email)}`, {status})
        .then((r) => { this.p_emails.find((e) => e.email === r.email).status = r.status; })
        .catch((e) => { this.on_error("Error when upading email status", e); });
      } catch(e) {
        this.on_error("Error in set_status", e);
      }
    },
    on_error: function(message, error) {
      this.$emit('on_error', { message, error });
    }
  }
};
</script>
