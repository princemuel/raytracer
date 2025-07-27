# Ray Trace Challenge

This is a repository I made for watching a React Mock Interview via Youtube.
I watched it first then tried to solve the problem on my own.
The website is not responsive.

## Table of Contents

- [Challenge](#challenge)
- [Screenshot](#screenshot)
- [Link](#link)
- [What I Learned](#what-i-learned)
- [Further Development](#further-development)
- [Resources](#resources)
- [Author](#author)

## Challenge

![Screen Shot 2022-10-11 at 1 22 19 PM](https://user-images.githubusercontent.com/89284873/195185587-477f0c26-ef48-4391-b158-9eb30156b5ad.png)

- Create 6 cute kitten cards from the data provided
- Year of age should be correct
- Create cards using grid

## Screenshot

![Screen Shot 2022-10-11 at 3 00 39 PM](https://user-images.githubusercontent.com/89284873/195187377-0b5cdab7-7ebc-427b-8499-7be7463f766f.png)

## Link

- [Mock Junior Front End Web Developer Interview with Mike Chen and Silvia](https://www.youtube.com/watch?v=j7CG7awlrQA)

## What I learned

- It's always good to practice and voice out your thought patterns while coding
- Never be afraid to ask questions when you get stuck
- Use good HTML semantics
- Application of slice method on a real project:

```js
const MAX_CATS = 6;
const App = () => {
  return (
    <section className="App">
      {CATS.slice(0, MAX_CATS).map((cat) => {
        return <Card {...cat} />;
      })}
    </section>
  );
};
```

## Further Development

- You can make the website responsive by collapsing the grid
- More UX/UI design
- Use API for more or real live data

## Resources

- [CSS Grid Layout](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Grid_Layout)
- [React Docs](https://beta.reactjs.org)

## Author

- Website - [Ash Moreno](https://www.ashmoreno.dev)
- Twitter - [@sexy_gravy](https://twitter.com/sexy_gravy)
