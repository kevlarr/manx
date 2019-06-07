<template>
  <div class="MessageInput">
    <TextArea
      placeholder="Say something..."
      v-model="value"
      @shift-enter="submit"
      @escape="clear"
    />
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';

@Component
export default class MessageInput extends Vue {
  value: string = '';

  submit() {
    this.$emit('submit', this.value);
    this.clear();
  }

  clear() {
    this.value = '';
  }
}
</script>

<style lang='scss'>
.MessageInput {
  & > .TextArea {
    height: 100%;

    & > textarea {
      height: 100%;
    }
  }
}
</style>
