import { mount } from "@vue/test-utils";
import SidebarItem from './SidebarItem.vue';

describe('components/SidebarItem.vue', () => {
  it('renders', async () => {
    const wrapper = mount(SidebarItem, {
      props: { current: false, disabled: false },
      slots: {
        default: 'A test slot',
        icon: 'A test icon slot',
      }
    });
    expect(wrapper.find('#sidebaritem-a-content').text()).toContain('A test slot');
    expect(wrapper.find('#sidebaritem-div-icon').text()).toContain('A test icon slot');
  });
});
