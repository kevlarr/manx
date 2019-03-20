<template>
  <div class="TextInput">
    <label v-if="label" :for="id" :class="{ isFocused, hasError }">{{ label }} {{ errMsg }}</label>
    <input type="text"
      v-bind="{ autofocus, disabled, id, maxlength, minlength, name, pattern, placeholder, value }"
      :class="[classNames, { isFocused, hasError }]"
      @input="$emit('input', $event.target.value)"
      @focus="setActive"
      @blur="setBlur"
    />
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';

@Component({
  props: {
    value: { type: String, required: true },
    label: String,
    errMsg: String,

    // Input properties
    id: String,
    classNames: String,
    autofocus: Boolean,
    minlength: Number,
    maxlength: Number,
    name: String,
    pattern: String,
    placeholder: String,
    disabled: Boolean,
  },
})
export default class extends Vue {
  focused: Boolean = false;

  get isFocused() { return this.focused; }
  get hasError() { return !!this.$props.errMsg; }

  setActive() {
    this.focused = true;
  }

  setBlur() {
    this.focused = false;
  }
};
</script>

<style scoped lang='scss'>
@import '@/styles/colors.scss';

.TextInput {
  margin-bottom: 10px;
  text-align: left;

  label {
    background: #ccc;
    border-radius: 3px;
    display: inline-block;
    font-size: 14px;
    padding: 0 6px;
    transform: translatey(40%) translatex(8px);
    transition: background 0.25s;

    &.isFocused {
      background: $lightBlue;
      transition: background 0.25s;
    }

    &.hasError {
      background: $lightRed;
      transition: background 0.25s;

      &.isFocused {
        background: $midRed;
      }
    }
  }

  input {
    border: 1px solid #ccc;
    border-radius: 3px;
    display: block;
    font-size: 16px;
    padding: 10px 10px 5px 10px;
    transition: border-color 0.25s;
    width: 100%;

    // box-shadow: 0px 0px 30px 15px rgba($midBlue, .1);


    &.isFocused {
      border-color: $lightBlue;
      transition: border-color 0.25s;
    }

    &.hasError {
      border-color: $lightRed;
      transition: border-color 0.25s;

      &.isFocused {
        border-color: $midRed;
      }
    }
  }
}
</style>
