// This is an implementation of Rust's state-type pattern from https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html

type Post =
  | DraftPost
  | InReviewPost
  | ApprovedPost
  | RejectedPost
  | PublishedPost;

type DraftPost = {
  _content: string;
  createdOn: Date;
};

type InReviewPost = {
  _content: string;
  reviewdOn: Date;
};

type RejectedPost = {
  redacted: string;
  rejectedOn: Date;
};

type ApprovedPost = {
  _content: string;
  approvedOn: Date;
};

type PublishedPost = {
  content: string;
  publishedOn: Date;
};

const draftPost = (content: string): DraftPost => ({
  _content: content,
  createdOn: new Date(),
});

const reviewDraft = (p: DraftPost): InReviewPost => ({
  ...p,
  reviewdOn: new Date(),
});

const rejectInReview = (p: InReviewPost): RejectedPost => ({
  ...p,
  redacted: p._content,
  rejectedOn: new Date(),
});

const approveInReview = (p: InReviewPost): ApprovedPost => ({
  ...p,
  approvedOn: new Date(),
});

const publishApproved = (p: ApprovedPost): PublishedPost => ({
  ...p,
  content: p._content,
  publishedOn: new Date(),
});

//------------//

let post: Post = draftPost("Some new post");
console.log(post);

// Cannot call any other function except reviewDraft
// Since no other function accepts this type
post = reviewDraft(post);
console.log(post);

// Can only approve or reject an in-review post
post = approveInReview(post);
console.log(post);

// Now content is public on publishing
post = publishApproved(post);
console.log(post);

// Note it is impossible to have invalid states,
// for eg cannot publish a draft without approval first:
// approveInReview(draftPost("Some post")) // uncomment to see error
