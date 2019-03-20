<template>
  <button class="Button" v-bind="{ class: classNames, disabled, type }" @click="$emit('click')">
    <slot></slot>
  </button>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';

@Component({
  props: {
    type: String,
    classNames: String,
    disabled: Boolean,
  },
})
export default class extends Vue {
}
</script>

<style scoped lang='scss'>
@import '@/styles/colors.scss';

.Button {
  background: $lightestGrey;
  border: 0;
  border-radius: 1px;
  cursor: pointer;
  font-size: 16px;
  padding: 10px 20px;
  margin: 0px 5px;
  transition: background 0.25s;

  &:hover {
    background: $lightGrey;
    transition: background 0.25s;
  }

  &.primary {
    background: $midBlue;
    color: white;

    &:hover {
      background: $darkBlue;
    }
  }

  &:disabled {
    background: $lightestGrey;
    color: $lightGrey;
    cursor: not-allowed;

    &:hover {
      background: $lightestGrey;
    }
  }
}
</style>
