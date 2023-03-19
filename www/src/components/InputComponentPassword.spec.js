import { mount } from '@vue/test-utils';
import { nextTick } from 'vue';
import InputComponentPassword from './InputComponentPassword.vue';
import InputComponent from './InputComponent.vue';

describe('components/InputComponentPassword.vue', () => {
  it('renders', async () => {
    const wrapper = mount(InputComponentPassword, {
      props: {
        label: 'test label',
        modelValue: 'P@ssword1234',
        disabled: false,
      }
    });
    expect(wrapper.findAll('#inputcomponentpassword-p-require').length).toEqual(0);
    const input_component_wrapper = wrapper.findComponent(InputComponent);

    await input_component_wrapper.find('#input').setValue('Pass');
    expect(wrapper.emitted('update:modelValue')[0][0]).toEqual('Pass');
    expect(wrapper.emitted('on_password_change_valid')[0][0]).toEqual(false);

    await input_component_wrapper.find('#input').setValue('Password1234');
    expect(wrapper.emitted('update:modelValue')[1][0]).toEqual('Password1234');
    expect(wrapper.emitted('on_password_change_valid')[1][0]).toEqual(false);

    await input_component_wrapper.find('#input').setValue('P@ssword1234');
    expect(wrapper.emitted('update:modelValue')[2][0]).toEqual('P@ssword1234');
    expect(wrapper.emitted('on_password_change_valid')[2][0]).toEqual(true);

    await input_component_wrapper.find('#input').setValue('p@ssword1234');
    expect(wrapper.emitted('update:modelValue')[3][0]).toEqual('p@ssword1234');
    expect(wrapper.emitted('on_password_change_valid')[3][0]).toEqual(false);

    await input_component_wrapper.find('#input').setValue('P@O4');
    expect(wrapper.emitted('update:modelValue')[4][0]).toEqual('P@O4');
    expect(wrapper.emitted('on_password_change_valid')[4][0]).toEqual(false);
  });

  it('renders with a notification for incomplete password', async () => {
    const wrapper = mount(InputComponentPassword, {
      props: {
        label: 'test label',
        modelValue: 'P@o4',
        disabled: false,
      }
    });
    expect(wrapper.find('#inputcomponentpassword-p-require').text()).toContain('Require');
  });
});
