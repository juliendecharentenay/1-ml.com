import { mount } from '@vue/test-utils';
import ErrorAlert from './ErrorAlert.vue';

describe('components/ErrorAlert.vue', () => {
  it('renders', async () => {
    const wrapper = mount(ErrorAlert, {
      props: {
        message: "A test error",
        error: null,
      },
    });
    expect(wrapper.find('#erroralert-p-message').text()).toContain("A test error");

    await wrapper.find('#erroralert-button-dismiss').trigger('click');
    expect(wrapper.emitted().dismiss).toHaveLength(1);

  });
});
