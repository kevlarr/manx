<template>
  <div class="Comment">
    <div class="content" v-html="comment.rendered"></div>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import { Observer } from 'mobx-vue';
import { Comment as CommentModel } from '@/types';

@Observer
@Component({
  props: {
    comment: Object as () => CommentModel,
  },
})
export default class Comment extends Vue {
}
</script>

<style lang='scss'>
.Comment {
  padding: 10px;
}

.content {
  p {
    margin: 0;
  }

  code {
    background: #333;
    color: #fff;
    display: inline-block;
    font-size: 16px;
    padding: 1px 4px;
    transform: translatey(-1px);
  }

  pre {
    background: #333;
    color: #fff;
    padding: 4px 8px;
  }

  pre code {
    padding: 0;
  }
}
</style>
