document.addEventListener('DOMContentLoaded', async () => {
    // Fetch and render blog posts on page load
    await fetchAndRenderPosts();
  
    // Handle form submission for creating new posts
    const form = document.getElementById('blogPostForm');
    form.addEventListener('submit', async (event) => {
      event.preventDefault(); // Prevent default form submission
  
      const text = document.getElementById('postContent').value;
      const username = document.getElementById('username').value;
      const avatarPath = document.getElementById('avatarUrl').value;
      const postImagePath = document.getElementById('postImage').value;
      const publication_date = new Date().toISOString().split('T')[0]; // Get current date in YYYY-MM-DD format
  
      const newPost = {
        text,
        username,
        publication_date,
        avatar_path: avatarPath || null,
        post_image_path: postImagePath || null
      };
  
      try {
        const response = await fetch('http://localhost:3000/blogpost', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(newPost),
        });
  
        if (!response.ok) {
          throw new Error(`An error occurred: ${response.statusText}`);
        }
  
        // Refresh posts after successful creation
        await fetchAndRenderPosts();
  
        // Clear the form
        form.reset();
      } catch (error) {
        console.error('Error creating post:', error);
      }
    });
  });
  
  // Function to fetch and render blog posts
  async function fetchAndRenderPosts() {
    try {
      const response = await fetch('http://localhost:3000/blogposts');
  
      if (!response.ok) {
        throw new Error(`An error occurred: ${response.statusText}`);
      }
  
      const blogPosts = await response.json();
      console.log('Blog Posts:', blogPosts);
  
      // Render blog posts
      renderBlogPosts(blogPosts);
    } catch (error) {
      console.error('Error fetching blog posts:', error);
    }
  }
  
  // Function to render blog posts as cards
  function renderBlogPosts(blogPosts) {
    const container = document.getElementById('blogPostsContainer');
    container.innerHTML = ''; // Clear any existing content
  
    blogPosts.forEach(post => {
      const card = document.createElement('div');
      card.classList.add('card');
  
      const publicationDate = new Date(post.publication_date);
      const formattedDate = publicationDate.toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
      });
  
      card.innerHTML = `
        ${post.avatar_path ? `<img src="${post.avatar_path}" alt="Avatar" class="avatar">` : ``}
        <h3>${post.username}</h3>
        <p class="publication-date">Published on ${formattedDate}</p>
        <p>${post.text}</p>
        ${post.post_image_path ? `<img src="${post.post_image_path}" alt="Post Image" class="post-image">` : ''}
      `;
  
      container.appendChild(card);
    });
  }
  