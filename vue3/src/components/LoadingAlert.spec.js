import { mount } from "@vue/test-utils";
import LoadingAlert from './LoadingAlert.vue';

describe('components/LoadingAlert.vue', () => {
  it('renders', () => {
    const wrapper = mount(LoadingAlert, {
      props: {
        message: "A test message"
      }
    });
    expect(wrapper.find('#loadingalert-p-message').text()).toContain("A test message");
  });
});

