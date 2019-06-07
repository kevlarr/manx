<template>
  <div class="CommentList">
    <Comment v-for="c in comments" :key="c.id" :comment="c"/>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import { Observer } from 'mobx-vue';
import Comment from '@/components/comments/Comment.vue';
import { Comment as CommentModel } from '@/types';

@Observer
@Component({
  components: {
    Comment,
  },
  props: {
    comments: Array as () => CommentModel[],
  },
})
export default class CommentList extends Vue {
}
</script>

<style lang='scss'>
.CommentList {
  overflow: auto;

  .Comment {
    margin: 10px 0;
  }
}
</style>
